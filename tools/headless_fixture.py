"""Helpers for reliable command output and headless mob-state inspection."""

import json
import re


class HeadlessCommands:
    """Read command-scoped chat rows without guessing across identical replies."""

    def __init__(self, control):
        self.control = control

    def __call__(self, command):
        rows = self.control("command", command).get("chat", [])
        return rows

    def mobs(self):
        rows = self("naturalist_fixture_mobs")
        marker = "Mob fixture "
        payload = next((row.split(marker, 1)[1] for row in rows if marker in row), None)
        if payload is None:
            raise RuntimeError(f"Headless mob state fixture returned no payload: {rows!r}")
        return json.loads(payload)

    def entity(self, selector):
        match = re.search(r"type=([^,\]]+)", selector)
        if match is None:
            raise RuntimeError(f"Cannot resolve entity selector through mob state fixture: {selector!r}")
        kind = match.group(1)
        tag = re.search(r"tag=([^,\],]+)", selector)
        required_tag = tag.group(1) if tag else None
        nbt_variant = re.search(r'Variant:"([^"}]+)"', selector)
        nbt_from_bucket = re.search(r"FromBucket:(1b|0b)", selector)
        mob = next((mob for mob in self.mobs()["mobs"] if mob["alive"] and mob["kind"] == kind
                    and (required_tag is None or required_tag in mob.get("tags", []))
                    and (nbt_variant is None or mob.get("variant") == nbt_variant.group(1))
                    and (nbt_from_bucket is None or mob.get("from_bucket", False) == (nbt_from_bucket.group(1) == "1b"))), None)
        if mob is None:
            raise RuntimeError(f"No live {kind} matching {selector!r} in headless mob state")
        return mob

    def entity_data(self, selector, field):
        mob = self.entity(selector)
        if field == "Health":
            return f"{mob['health']}f"
        if field == "Air":
            return f"{mob['air']}s"
        if field == "Age":
            return f"{mob['age']}"
        if field == "Variant":
            return f'"{mob["variant"]}"' if mob["variant"] else "<missing>"
        if field == "Gray":
            return "1b" if str(mob["gray"]).endswith("(1)") else "0b"
        if field == "ConversionTime":
            return str(mob["conversion_time"])
        if field == "HuntingCooldown":
            return str(mob["hunting_cooldown"])
        if field == "FromBucket":
            return "1b" if mob.get("from_bucket", False) else "0b"
        raise RuntimeError(f"Headless observer does not expose {field!r}")
