from os import walk
from pathlib import Path
from setuptools import find_packages, setup


def get_version():
  with open("VERSION", "rt") as f:
    return f.readline().strip()


def init_package():
  package="build/generated/source/proto/main/python"
  for root, dirs, _ in walk(package):
    for d in dirs:
      Path(root, d, "__init__.py").touch()
  return {"packages": find_packages(package),
          "package_dir": {"": package}}


setup(
  name="tensorflow_serving_client",
  python_requires=">=3.5",
  install_requires=["grpcio", "protobuf"],
  author="Figroc Chen",
  author_email="figroc@gmail.com",
  license="Apache License 2.0",
  url="https://github.com/figroc/tensorflow-serving-client",
  version=get_version(),
  **init_package(),
)
