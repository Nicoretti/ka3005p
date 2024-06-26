# TODO: Fix outdated API bits
from typing import List, Optional, Any, Union

class PowerSupply:

    def __init__(self, serial_port: Optional[str] = None) -> None: ...

    @staticmethod
    def list_power_supplies() -> List[str]: ...

    def execute(self, command: str) -> List[int]: ...

    @property
    def current(self) -> float: ...

    @current.setter
    def current(self, i: float) -> None: ...

    @property
    def voltage(self) -> float: ...

    @voltage.setter
    def voltage(self, v: float) -> None: ...

    @property
    def status(self) -> str: ...

    @property
    def on(self) -> bool: ...

    @on.setter
    def on(self, enable: bool) -> None: ...

    @property
    def off(self) -> bool: ...

    @off.setter
    def off(self, disable: bool) -> None: ...

    @beep.setter
    def beep(self, enable: bool) -> None: ...

    def save(self, id: int) -> None: ...

    def load(self, id: int) -> None: ...

    @ocp.setter
    def ocp(self, enable: bool) -> None: ...

    @ovp.setter
    def ovp(self, enable: bool) -> None: ...
