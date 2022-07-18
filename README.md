# CopperNotes

_The rustacean version of GopherNotes, written both to learn and to enhance the original usecase_

I realized while trying to use my own tool that I would rarely leverage it's basic functionality without a specific use-case. That usecase was typically piping terminal output, code, and small notes into a specific "note" to keep track of various clippings I had essentially thought needed to be grouped together.

That got me thinking, why not update GopherNotes to support this use-case instead of it's current? Surely, if you had identified friction in your own dogfooding, others would have as well right? I'll do you one better kind reader. Let's rewrite it as CopperNotes in this hyped up language that I've seen all over the interwebs. That way, we can compare experiences, and also not have to rewrite Go code when the schema changes. Good idea you say.

## Installation

## Usage

| Function   | Command                                     | Example                                                                          | Comments & Tips                                                                       |
| ---------- | ------------------------------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| New Ticket | `cn -t <TICKET_NUMBER>`                     | `cn -t coppernotes-01`                                                           | This will create the ticket if said ticket doesn't exist                              |
| New Note   | `cn -t <TICKET_NUMBER> -n "<NOTE_CONTENT>"` | `cn -t coppernotes-01 -n "I really hope this doesn't become a flamewar example"` | This will create the note associated to the ticket if the content provided is unique. |
