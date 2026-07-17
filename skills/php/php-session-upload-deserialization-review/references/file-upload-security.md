# File upload security

## Why this matters

An upload handler is the one place in most PHP applications where a client
supplies content the server will store, and often later serve, on its own
filesystem. Every trust decision the manual warns against — believing the
client's stated MIME type, believing the client's filename, skipping the
configured size/count ceilings, or placing accepted content somewhere the
web server will execute it — turns a routine upload feature into a path for
an attacker to plant and run a web shell, exhaust server resources, or read
files the application never meant to expose.

## NORMATIVE: configuration ceilings gate every upload

Per the php.net manual's common-pitfalls guidance for file uploads:

- **`upload_max_filesize`** caps the size PHP will accept per file: "The
  `MAX_FILE_SIZE` item cannot specify a file size greater than the file size
  that has been set in the `upload_max_filesize` in the php.ini file. The
  default is 2 megabytes." A form-field `MAX_FILE_SIZE` hint is a
  client-side convenience only; the server-enforced ceiling is
  `upload_max_filesize`.
- **`post_max_size`** must accommodate the request as a whole: "If
  `post_max_size` is set too small, large files cannot be uploaded. Make
  sure you set `post_max_size` large enough." A `post_max_size` smaller than
  `upload_max_filesize` silently caps effective upload size below what
  `upload_max_filesize` alone suggests.
- **`max_file_uploads`** bounds how many files one request can carry: "The
  `max_file_uploads` configuration setting controls the maximum number of
  files that can [be] uploaded in one request. If more files are uploaded
  than the limit, then `$_FILES` will stop processing files once the limit
  is reached" — silently, not with an error the handler is guaranteed to
  check for.
- **`memory_limit`**, **`max_execution_time`**, and **`max_input_time`** must
  also be sized for uploads: the manual specifically notes `max_input_time`
  "sets the maximum time, in seconds, the script is allowed to receive
  input; this includes file uploads. For large or multiple files, or users
  on slower connections, the default of `60` seconds may be exceeded,"
  causing an otherwise-valid upload to fail partway through.

A review must confirm these are set deliberately (not left at silent
defaults that either reject legitimate uploads or, in the case of
`max_file_uploads`, silently truncate a batch without the handler noticing).

## NORMATIVE: do not trust which file you operate on

The manual's common-pitfalls page states plainly: "Not validating which file
you operate on may mean that users can access sensitive information in
other directories." This is the manual's own framing of path-traversal and
client-trust risk in upload handling — a filename or path is untrusted input
like any other and must be validated, not built into a filesystem operation
directly. The manual separately cautions that "due to the large amount of
directory listing styles we cannot guarantee that files with exotic names
(like containing spaces) are handled properly," reinforcing that the
client-supplied filename cannot be relied on to behave predictably and
should not be used to construct a storage path or to make a security
decision (such as inferring file type from its extension).

## RECOMMENDATION: validate content, not client claims; store outside the webroot

The manual's guidance above establishes the principle — validate the file
you actually operate on — but does not itself specify a validation
algorithm or a storage location; the following are this skill's operating
recommendations applying that principle, to be checked as reviewer evidence
criteria below rather than cited as separate manual quotations:

- Treat `$_FILES[...]['type']` (client-reported MIME type) and
  `$_FILES[...]['name']` (client-reported filename) as untrusted metadata
  only. A security decision — what kind of file this is, whether it is
  acceptable — must be made from the file's actual content on the server,
  not from either client-supplied field.
- Store accepted uploads outside any path the web server will serve
  directly, and generate the stored filename server-side rather than
  reusing the client-supplied name verbatim, closing both the
  path-traversal risk above and the risk of an executable extension being
  preserved into a servable location.
- Never `include()`, `require()`, or otherwise execute an uploaded file as
  PHP code, and never configure the storage directory to be
  script-executable by the web server.

## Reviewer evidence criteria

For each upload handler in scope:

- Confirm `upload_max_filesize`, `post_max_size` (sized larger than
  `upload_max_filesize`), and `max_file_uploads` are set to deliberate,
  documented values rather than left at framework/PHP defaults without
  review, and confirm the handler does not assume a client-side
  `MAX_FILE_SIZE` hint is enforced.
- Confirm the handler checks the actual number of files processed against
  what the client claims to have sent, given that `$_FILES` silently stops
  populating once `max_file_uploads` is reached.
- Confirm the handler validates the file's real type/content server-side,
  not `$_FILES[...]['type']` or the client filename's extension alone.
- Confirm accepted files are stored outside any web-servable path, with a
  server-generated filename, not the client-supplied one used verbatim in
  a filesystem path.
- Confirm no code path `include()`s, `require()`s, or otherwise executes an
  uploaded file, and that the storage location is not configured as
  script-executable.
- Treat trusting client-supplied MIME type/filename for a security decision,
  storing inside the webroot, or having no enforced size/type/count limit as
  a blocking finding.

## Applicable versions

The `upload_max_filesize` default (2 megabytes) and the `max_file_uploads`
silent-truncation behavior described above are current php.net manual
guidance as of this review; re-verify against the live manual page before
citing an exact default value, since PHP distribution defaults (and
distro-specific `php.ini` overrides) can differ from the documented
upstream default.

## Sources

- [PHP Manual — Common Pitfalls (file uploads)](https://www.php.net/manual/en/features.file-upload.common-pitfalls.php) — supports the `upload_max_filesize` default and `MAX_FILE_SIZE` client-hint framing, the `post_max_size` sizing guidance, the `max_file_uploads` silent-stop behavior, the `max_input_time` upload-duration caution, and the "not validating which file you operate on" and "exotic names" client-trust cautions.

Last verified: 2026-07-16.
