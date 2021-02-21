import os
from xml.etree import ElementTree

import fire

workspace_path = os.path.join(os.path.dirname(__file__), ".idea", "workspace.xml")

_config_template = """
<configuration name="test [{name}-{label}]" type="CargoCommandRunConfiguration" factoryName="Cargo Command" temporary="true">
    <option name="command" value="compete test {name}-{label}" />
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


def _make_configuration_elm(name, label):
    return ElementTree.XML(_config_template.format(name=name, label=label))


def run(name):
    labels = [f[0] for f in sorted(os.listdir(os.path.join(os.path.dirname(__file__), name, "src", "bin")))]
    if len(labels) == 0:
        raise FileNotFoundError

    tree = ElementTree.parse(workspace_path)
    root = tree.getroot()
    for component in root.iterfind("component"):
        if component.get("name") == "CargoProjects":
            elm = ElementTree.Element("cargoProject", attrib={"FILE": f"$PROJECT_DIR$/{name}/Cargo.toml"})
            component.append(elm)
        if component.get("name") == "RunManager":
            for lb in labels:
                component.append(_make_configuration_elm(name, lb))
    tree.write(workspace_path)


if __name__ == '__main__':
    fire.Fire(run)
