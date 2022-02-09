# ScribeDown
## Beautiful, Clean, Writer-Oriented

The goal of ScribeDown is to make Markdown the best, cleanest, nicest experience for those who are, like myself, using it to write books, blog posts, and other non-code-oriented things. Most Markdown editors are based on a split-view paradigm, and look like code editors on one side and web pages on the other, which really isn't pleasant for writing and proof-reading, and also isn't the norm for traditional word processing, which most people who are writing books and blog posts are typically more familiar with. ScribeDown changes all of this by orienting itself around a few core features:

### 1. What You See Is What You Get

Scribedown is WYSIWYG editor that uses Markdown under the hood as a highly-compatible back-end, but it tries to not expose that too much.

As you write Markdown formatted text, it displays that text with different font sizes, weights, indentation levels, etc, in order to give you a preview, inline, of what the final document will look like when rendered.

It also has keyboard controls similar to those used in traditional word processors to insert Markdown formatting, so you don't need to know Markdown at all.

Additionally, the text entry area in ScribeDown is centered, padded, and in a serif font, so it's designed to be as readable as possible, looking like an actual finished word-processing document instead of a code file.

### 2. Document-Oriented

ScribeDown lets you open an entire directory, so that all of the Markdown documents in that directory are put into your sidebar, to be opened in your current tab or in a new tab. Not only that, but instead of treating documents like code files, ScribeDown reads either the first H1 or the YAML `title` property from each Markdown file in your project directory, and uses that as the file's title.

### 3. Simple GUI Interface

Another focus for ScribeDown is having a clean, simple, beautiful user interface around your document and integrates well with the surrounding Desktop Environment. In order to do this, we have to sacrifice some platform interoperability, by choosing a DE to target. We chose GNOME.