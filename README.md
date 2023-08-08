I think most people use LaTeX simply to create a good looking document. However,
being a typesetting system, TeX and its derivatives forces users to meddle with
typesetting rather than focusing on the content, making it very unreadable.

So preText was originally conceived to create a programmable layer above LaTeX
with sensible defaults so users can focus on the content as much as possible:

```
// sections.ptx
let problem_section: start_year, fastforward, adam, sister {
  let catchphrase = "Being dumb is so <start_year>."

  return("
    In the year <start_year>, Adam is <adam> years old and his sister is
    <adam/sister> his age. Therefore, in the year <start_year+fastforward>
    Adam will be <adam+fastforward> and his sister is <sister+fastforward>. I
    am <bold('very')> smart. <catchphrase()>
  ");
}

let abstract = "This problem has been haunting humanity since the dawn of
  civilization. In this paper, a perfect solution is introduced."
```

For pure content files, much of the syntax can be left out since it is treated as a simple section identified by the filename:

```
// content.ptx
<abstract()>
I solved the world's biggest problem:
<sections.problem_section(2010, 20, 10, 5)>

Proof left as an exercise to the reader
```

would be equivalent to

```
let content = {
  "<abstract()>
  I solved the world's biggest problem:
  <sections.problem_section(2010, 20, 10, 5)>

  Proof left as an exercise to the reader"
}
```

A main file glues it all together:

```
// main.ptx
import std.template.latex_article;
import "sections.ptx";
import "content.ptx";

latex_article(title="I am very smart (2010)", author="Mathematical Genius", body=content);
```

As you might have noticed, this idea can be applied to break any text files into modules, not just LaTeX.
