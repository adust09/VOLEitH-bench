# LaTeX compilation settings
# Output directory for all build files
$out_dir = 'build';

# Use XeLaTeX for Japanese support (as configured in main.tex)
$pdf_mode = 5;  # 5 = xelatex

# Ensure that auxiliary files also go to the output directory
$aux_dir = 'build';

# Enable continuous preview mode settings (optional)
$preview_continuous_mode = 0;

# Clean up temporary files
$clean_ext = 'synctex.gz synctex.gz(busy) run.xml tex.bak bbl bcf fdb_latexmk run tdo';

# Enable shell escape if needed for certain packages (use with caution)
# $xelatex = 'xelatex -shell-escape %O %S';
