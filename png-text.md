# png-text

PNG Text is a command line program which reads a binary (PNG) file,
and describes all PNG data in the file, regardless of whether or not
the data chunks form a complete valid PNG.

## CIS 542 project pitch
My idea for a project idea is to create a simple command line PNG metadata reader, similar to the Strings view on <https://fotoforensics.com>, which also searches for additional PNG data after the first IEND of the file.
(The latter is to examine the forensic data in overwritten but not truncated files, eg: <https://en.wikipedia.org/wiki/ACropalypse>.
This will not provide an actual renderer or GUI.
Search functionality will be provided by grep, or left out.
Any example images will be provided on an as-found/created basis (if I have them or need them to test the program) probably from wikipedia & co.

### Deliverables:
A command line program which produces a textual description of a PNG file, specifically of the metadata and any trailing data.
An as-needed collection of test PNG files.
User manual which describes the use and output format of the program.
A 7 min presentation (in class?).
Probably NOT a "results" document.
