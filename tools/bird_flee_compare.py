"""Compare headless Bird takeoff under Survival and Creative controls."""
import argparse
import json
from math import hypot
from pathlib import Path
from statistics import median


def read(path):
    rows = json.loads(path.read_text())
    assert len(rows) == 12, (path, len(rows))
    return rows


def summarize(rows):
    measurements = []
    for tick, points in zip((6, 24), zip(*rows)):
        altitudes = [round(y - 100, 5) for _, y, _ in points]
        measurements.append({
            "tick": tick,
            "minimum_rise": min(altitudes),
            "median_rise": median(altitudes),
            "maximum_rise": max(altitudes),
            "minimum_horizontal_retreat": round(min(hypot(x - 12.5, z - 16.5) - 4 for x, _, z in points), 5),
        })
    return measurements


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    for field in ("java_survival", "java_creative", "horizon_survival", "horizon_creative"):
        parser.add_argument(field, type=Path)
    args = parser.parse_args()
    report = {field: summarize(read(path)) for field, path in vars(args).items()}
    print(json.dumps(report, indent=2), flush=True)
    java_early, java_late = report["java_survival"]
    horizon_early, horizon_late = report["horizon_survival"]
    assert abs(horizon_early["median_rise"] - java_early["median_rise"]) <= 0.05, report
    assert horizon_early["minimum_horizontal_retreat"] >= 0.03, report
    assert horizon_late["minimum_horizontal_retreat"] >= 0.25, report
    assert horizon_early["minimum_rise"] >= java_early["minimum_rise"] - 0.1, report
    assert abs(horizon_late["median_rise"] - java_late["median_rise"]) <= 0.15, report
    assert report["java_creative"][0]["median_rise"] == report["horizon_creative"][0]["median_rise"] == 0, report
