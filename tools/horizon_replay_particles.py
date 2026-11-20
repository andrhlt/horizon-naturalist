"""Parse Horizon Studio recordings for vanilla particle packets in Minecraft 26.3."""
import json
import socket
import struct
import zipfile


def studio(socket_path, command):
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as stream:
        stream.settimeout(25)
        stream.connect(str(socket_path))
        stream.sendall((f"studio record {command}\n").encode())
        response = json.loads(stream.makefile().readline())
    assert response["ok"], response
    return response


def varint(data, start):
    value = 0
    for shift in range(0, 35, 7):
        byte = data[start]
        start += 1
        value |= (byte & 127) << shift
        if byte < 128:
            return value, start
    raise ValueError("Invalid variable-length integer in replay")


def read_particles(path, registry, ids):
    names = {row["protocol_id"]: name for name, row in registry["minecraft:particle_type"]["entries"].items()
             if name in ids}
    with zipfile.ZipFile(path) as archive:
        packets = archive.read("recording.tmcpr")
    start = 0
    while start < len(packets):
        _, length = struct.unpack_from(">II", packets, start)
        start += 8
        end = start + length
        packet_id, body = varint(packets, start)
        start = end
        if packet_id != 48:
            continue
        particle_id, body = varint(packets, body)
        if particle_id not in names:
            continue
        flags_and_geometry = struct.Struct(">??3d6f")
        limiter, always_show, *geometry = flags_and_geometry.unpack_from(packets, body)
        count, body = varint(packets, body + flags_and_geometry.size)
        randomization, body = varint(packets, body)
        assert body == end, (particle_id, body, end)
        yield {"name": names[particle_id], "count": count, "pos": geometry[:3],
               "spread": geometry[3:6], "speed": geometry[6:9], "limiter": limiter,
               "always_show": always_show, "randomization": randomization}

def read_sounds(path, registry, names):
    builtin_names = {row["protocol_id"]: name for name, row in registry["minecraft:sound_event"]["entries"].items()
                     if name in names}
    with zipfile.ZipFile(path) as archive:
        packets = archive.read("recording.tmcpr")
    start = 0
    while start < len(packets):
        _, length = struct.unpack_from(">II", packets, start)
        start += 8
        end = start + length
        packet_id, body = varint(packets, start)
        start = end
        if packet_id != 119:
            continue
        holder, body = varint(packets, body)
        if holder == 0:
            length, body = varint(packets, body)
            name = packets[body:body + length].decode()
            body += length
            fixed_range = bool(packets[body])
            body += 5 if fixed_range else 1
        else:
            name = builtin_names.get(holder - 1)
        if name not in names:
            continue
        source, body = varint(packets, body)
        x, y, z, volume, pitch, _seed = struct.unpack_from(">3i2fq", packets, body)
        assert body + struct.calcsize(">3i2fq") == end, (name, body, end)
        yield {"name": name, "source": source, "pos": [channel / 8 for channel in (x, y, z)],
               "volume": volume, "pitch": pitch}
