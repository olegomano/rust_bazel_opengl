import png
from array import array

INCLUDE_HEADER = """
extern crate asset;
"""

RS_TEMPLATE = """
pub const _asset : &'static asset::Asset= &asset::Asset{{
    name : "{name}",
    data : {var_name},
    width : {width},
    height : {height},
}};        
"""

BYTE_ARRAY_TEMPLATE = "const {var_name}: &'static [u8] = &[{hex_arr}];"

#generates a string that contains all assets backed into it
#static MY_STATIC_BYTE_ARRAY: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
def to_rs(var_name,path):

    reader = png.Reader(file=open(path,"rb"))
    width,height,img,info = reader.read()
    
    byte_count = width*height*4
    byte_array = bytearray([0x00] * byte_count)

    c = 0
    for row in img:
        for value in row:
            byte_array[c] = value
            c = c + 1

    byte_array = ','.join(['0x{:02X}'.format(byte) for byte in byte_array])

    array_decl = BYTE_ARRAY_TEMPLATE.format(
        var_name = var_name,
        id = None,
        hex_arr = byte_array,
    )

    struct_decl =  RS_TEMPLATE.format(
        var_name = var_name,
        name = var_name,
        width = width,
        height = height,
        id = ""
    )
    
    return INCLUDE_HEADER + "\n" + array_decl + "\n" + struct_decl
