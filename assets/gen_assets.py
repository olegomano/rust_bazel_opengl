import sys
import os
from typing import List
from dataclasses import dataclass
IMAGE_ASSET = "image"

@dataclass
class AssetClass:
    name : str
    ext : List = None
    folders : List = None


@dataclass
class Generator:
    gen : object = None
    asset_types : List  = None

@dataclass
class Context:
    working_dir : str
    asset_info : object
    generators : object


class AssetGenerator():
    def __init__(self,context):
        print(context.working_dir)
        self._context = context
        self._gen_lut = self._generators_for_asset_type(context)
        for _,asset_class in self._context.asset_info.items():
            self._handle_asset_class(asset_class)

    #generates all the asset files
    def run(self):
        pass

    #iterate the current folder and collect all the files
    def _collect_files(self,folder):
        all_files = []
        for root, _, files in os.walk(folder):
            for file in files:
                file_path = os.path.join(root, file)
                all_files.append(file_path)
        return all_files

    #for a given asset class find all the relevant files and generate the
    #resultant bazel files
    def _handle_asset_class(self,asset_class):
        files = []
        for folder in asset_class.folders:
            folder_path = os.path.join(self._context.working_dir, folder)
            files.extend(self._collect_files(folder_path))
        
        generators = self._gen_lut[asset_class.name]
        
        for file in files:
            for ext in asset_class.ext:
                if ext in file:
                    for gen in generators:
                        gen.gen(file)
    
    #generates a bazel file for the specified asset
    def _gen_bazel_file(self,asset):
        pass

    #generates a lut of asset_type to generator 
    #asset_tyoe -> [Generators]
    def _generators_for_asset_type(self,context):
        result = {}
        for generator in context.generators:
            for asset_type in generator.asset_types:
                if asset_type in context.asset_info:
                    if asset_type not in result:
                        result[asset_type] = []
                    result[asset_type].append(generator)
                else:
                    print("WARNING: Generator declared support for missing asset type " + str(asset_type))
        return result

def rust_generator(asset):
    #gen rust file that includes the asset
    #gen bazel file that exposes generated rust file
    print("Running rust generator")
    pass


#Runs the asset bazel generator, 
#pass in the target directory you want to generate the bazel file to
if __name__ == "__main__":
    print("Hello World")
    
    context = Context(
        working_dir = "/".join(sys.argv[0].split("/")[:-1]),
        asset_info = {
            IMAGE_ASSET : AssetClass(
                name = IMAGE_ASSET,
                ext = [".png"],
                folders = ["image"]
            )
        },
        generators = [
            Generator(
                gen = rust_generator,
                asset_types = [IMAGE_ASSET]
            ),
        ]
    )
    asset_generator = AssetGenerator(context) 
