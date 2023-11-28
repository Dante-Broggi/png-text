# png-text

PNG Text is a command line program which reads a binary (PNG) file,
and describes all PNG data in the file, regardless of whether or not
the data chunks form a complete valid PNG.

Specifically, the ouput format is a list of valid PNG chunk chains contained in 
the file, starting with those which begin with a valid PNG header, in order,
followed by those chunk chains which do not begin with a valid PNG header,
and then a list of bytes&byte offsets which do not form valid chunk chains.

After extracting loose IDAT chunks one could look for hidden pixels
either out of the declared width/height or in transparent pixels,
but this program does not do such image analysis, as this analusis was considered out of scope for the scale of a class project.
Look at the more developed <https://fotoforensics.com> / <https://lab.fotoforensics.com/?show=lab> service for such analysis, or at <https://en.wikipedia.org/wiki/ACropalypse> and <https://acropalypse.app> for the vulnerability which inspired this project.


## CIS 542 Computer Forensics class project pitch
My idea for a project idea is to create a simple command line PNG metadata reader, similar to the Strings view on <https://fotoforensics.com>, which also searches for additional PNG data after the first IEND of the file.
(The latter is to examine the forensic data in overwritten but not truncated files, eg: <https://en.wikipedia.org/wiki/ACropalypse>.
This will not provide an actual renderer or GUI.
Search functionality will be provided by grep, or left out.
Any example images will be provided on an as-found/created basis (if I have them or need them to test the program) probably from wikipedia & co.

### Class Deliverables:
A command line program which produces a textual description of a PNG file, specifically of the metadata and any trailing data.
An as-needed collection of test PNG files.
User manual which describes the use and output format of the program.
A 7 min presentation (in class?).
Probably NOT a "results" document.
