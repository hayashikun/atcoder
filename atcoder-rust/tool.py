import os
import subprocess
import time
from xml.etree import ElementTree

import fire

root_dir = os.path.dirname(__file__)
workspace_path = os.path.join(root_dir, ".idea", "workspace.xml")

_config_template = """
<configuration name="{cmd} [{name}-{label}]" type="CargoCommandRunConfiguration" factoryName="Cargo Command" temporary="true">
    <option name="command" value="compete {cmd} {name}-{label}" />
    <option name="workingDirectory" value="file://$PROJECT_DIR$/{name}" />
    <option name="channel" value="DEFAULT" />
    <option name="requiredFeatures" value="true" />
    <option name="allFeatures" value="false" />
    <option name="emulateTerminal" value="false" />
    <option name="backtrace" value="SHORT" />
    <envs />
    <option name="isRedirectInput" value="false" />
    <option name="redirectInputPath" value="" />
    <method v="2">
        <option name="CARGO.BUILD_TASK_PROVIDER" enabled="true" />
    </method>
</configuration>
"""


def _make_configuration_elm(cmd, name, label):
    return ElementTree.XML(_config_template.format(cmd=cmd, name=name, label=label))


def runner(name):
    labels = [
        str(f).split(".", 1)[0]
        for f in sorted(os.listdir(os.path.join(root_dir, name, "src", "bin")))
    ]

    if len(labels) == 0:
        raise FileNotFoundError

    tree = ElementTree.parse(workspace_path)
    root = tree.getroot()

    component_names = [c.get("name") for c in root.iterfind("component")]
    if "CargoProjects" not in component_names:
        elm = ElementTree.Element("component", attrib={"name": "CargoProjects"})
        root.append(elm)

    for component in root.iterfind("component"):
        if component.get("name") == "CargoProjects":
            elm = ElementTree.Element("cargoProject", attrib={"FILE": f"$PROJECT_DIR$/{name}/Cargo.toml"})
            component.append(elm)
        if component.get("name") == "RunManager":
            for lb in labels:
                component.append(_make_configuration_elm("submit", name, lb))
            for lb in labels:
                component.append(_make_configuration_elm("test", name, lb))

    tree.write(workspace_path)


def clean_xml():
    tree = ElementTree.parse(workspace_path)
    root = tree.getroot()
    for component in root.iterfind("component"):
        if component.get("name") == "CargoProjects":
            for elm in component.iterfind("cargoProject"):
                component.remove(elm)
        if component.get("name") == "RunManager":
            for elm in component.iterfind("configuration"):
                if elm.attrib["type"] == "CargoCommandRunConfiguration" \
                        and elm.attrib.get("temporary", "false") == "true":
                    component.remove(elm)
    tree.write(workspace_path)


def clean():
    for _ in range(4):
        clean_xml()
        time.sleep(0.5)


def new(name):
    subprocess.run(["cargo", "compete", "new", name])
    runner(name)


def todo():
    for d in sorted(os.listdir(root_dir)):
        if not os.path.exists(os.path.join(root_dir, d, "Cargo.toml")):
            continue
        bin_dir = os.path.join(root_dir, d, "src", "bin")
        for rs in os.listdir(bin_dir):
            with open(os.path.join(bin_dir, rs)) as f:
                line = f.readline()
            if "TODO" in line:
                print(f"[{d}-{rs.split('.')[0]}]: {os.path.join(bin_dir, rs)}")


if __name__ == '__main__':
    fire.Fire({
        "runner": runner,
        "clean": clean,
        "new": new,
        "todo": todo
    })
