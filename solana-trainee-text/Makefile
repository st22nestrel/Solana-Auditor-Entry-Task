# Makefile for LaTeX project with minted package

# Compiler options
TEX = pdflatex
BIB = bibtex

# Main LaTeX file
MAIN = main.tex

# Final file name
PROJ = wormhole-report

# LaTeX compiler flags
TEXFLAGS = -shell-escape

# Build target
all: $(PROJ).pdf

# Compile LaTeX file
$(PROJ).pdf: $(MAIN)
	$(TEX) $(TEXFLAGS) -jobname=$(PROJ) $<
	$(BIB) $(PROJ)
	$(TEX) $(TEXFLAGS) -jobname=$(PROJ) $<
	$(TEX) $(TEXFLAGS) -jobname=$(PROJ) $<

# Clean auxiliary files
clean:
	rm -f $(PROJ).aux $(PROJ).bbl $(PROJ).blg $(PROJ).log $(PROJ).out $(PROJ).pyg
