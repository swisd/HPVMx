# 1. Define the base path
$basePath = ".\target\doc\HPVMx"

# 2. Get all index.html files, sorted by depth so parent modules come before submodules
$htmlFiles = Get-ChildItem -Path $basePath -Filter "index.html" -Recurse |
        Sort-Object { $_.FullName.Split('\').Count }

# 3. Run Pandoc to merge them into one Word Doc
# We use --toc to generate the Table of Contents automatically
pandoc ($htmlFiles.FullName) -o HPVMx_Full_Manual.docx --toc --toc-depth=3 --metadata title="HPVMx Documentation"