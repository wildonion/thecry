

import zipfile

with zipfile.ZipFile('final.xmdx', 'r') as zip_ref:
    zip_ref.extractall('extracted_folder')