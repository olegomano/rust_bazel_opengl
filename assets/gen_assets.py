import sys
import os
import img_loader
from typing import List
from dataclasses import dataclass
IMAGE_ASSET = "image"

@dataclass
class Asset:
    id : int 
    path : str
    name : str
    type : object
    failed_gen : bool = False


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
    output_dir : str
    asset_info : object
    generators : object


class AssetGenerator():
    def __init__(self,context):
        print(context.working_dir)
        self._asset_id = 0
        
        self._context = context
        self._gen_lut = self._generators_for_asset_type(context)
        self._assets = self._discover_assets()
        print("Discovered " + str(len(self._assets)) + " assets")

        processed_assets = 0
        for asset in self._assets:
            for gen in self._gen_lut[asset.type]:
                try:
                    gen.gen(self,asset)
                except Exception as e:
                    print(e)
                    asset.failed_gen = True
                processed_assets += 1
                print("{}/{}".format(processed_assets , len(self._assets)))
        
        self._save_file("gen/lib.rs",self._gen_asset_lib_rs())

    #traverse the generators and see what assets they want to use
    def _discover_assets(self):
        result = []

        for gen in self._context.generators:
            for asset_type in gen.asset_types:
                for folder in self._context.asset_info[asset_type].folders:
                    for file in self._collect_files(os.path.join(self._context.working_dir,folder)):
                        self._asset_id = self._asset_id + 1
                        asset = Asset(
                            id = self._asset_id,
                            path = file,
                            name = file[len(self._context.working_dir):].replace("/","_").replace(" ","_").replace(".","_"),
                            type = asset_type,
                        )
                        result.append(asset)
        return result 

    #iterate the current folder and collect all the files
    def _collect_files(self,folder):
        all_files = []
        for root, _, files in os.walk(folder):
            for file in files:
                file_path = os.path.join(root, file)
                all_files.append(file_path)
        return all_files
 

    def _gen_asset_lib_rs(self):
        MOD_TEMPLATE = "pub mod {};\n"
        result = ""
        for asset in self._assets:
            if not asset.failed_gen:
                result+= MOD_TEMPLATE.format(asset.name)
        return result
   
    #saves a string to the following path relative to the bazel root
    def _save_file(self,path,data):
        path = os.path.join(self._context.output_dir,path)
        print("writing " + path)
        with open(path,"w") as file:
            file.write(data)

    #generates a lut of asset_type to generator 
    #asset_tyoe -> [Generators]
    def _generators_for_asset_type(self,context):
        result = {}
        count = 0
        for generator in context.generators:
            for asset_type in generator.asset_types:
                if asset_type in context.asset_info:
                    if asset_type not in result:
                        result[asset_type] = []
                    result[asset_type].append(generator)
                else:
                    print("WARNING: Generator declared support for missing asset type " + str(asset_type))
        return result



#generates 
def img_generator_rust(generator,asset):
    #gen rust file that includes the asset
    #gen bazel file that exposes generated rust file
    rs_code = img_loader.to_rs(asset.name,asset.path)  
    out_path = "gen/" + asset.name + ".rs"
    generator._save_file(out_path,rs_code)


#Runs the asset bazel generator, 
#pass in the target directory you want to generate the bazel file to
if __name__ == "__main__":
    context = Context(
        working_dir = "/".join(sys.argv[0].split("/")[:-1]),
        output_dir = "/home/oleg/Documents/Dev/Galaga/assets", #TODO(oleg): pass in output dir as env var
        asset_info = {
            IMAGE_ASSET : AssetClass(
                name = IMAGE_ASSET,
                ext = [".png"],
                folders = ["image"]
            ),
        },
        generators = [
            Generator(
                gen = img_generator_rust,
                asset_types = [IMAGE_ASSET]
            ),
        ]
    )
    asset_generator = AssetGenerator(context) 
