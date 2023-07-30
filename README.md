I think most people use LaTeX simply to create a good looking document. However,
being a typesetting system, TeX and its derivatives forces users to meddle with
typesetting rather than focusing on the content, making it very unreadable.

So preText was originally conceived to create a programmable layer above LaTeX
with sensible defaults so users can focus on the content as much as possible:

```
  // problem.ptx
  import std.bold;

  let problem_section: start_year, fastforward, adam, sister {
    let catchphrase = "Being dumb is so <start_year>."

    return("
      In the year <start_year>, Adam is <adam> years old and his sister is
      <adam/sister> his age. Therefore, in the year <start_year+fastforward>
      Adam will be <adam+fastforward> and his sister is <sister+fastforward>. I
      am <bold('very')> smart. <catchphrase()>
    ");
  }

  // Optional return if it's the only statement
  let abstract {
    "This problem has been haunting humanity since the dawn of civilization. In this paper, a perfect solution
    is introduced."
  }
```

This section then can be reused in another file.

```
  // main.ptx
  import std.template.latex_article;

  return(
    latex_article(title="I am very smart (2010)", author="Mathematical Genius", body="
      <abstract()>
      I solved the world's biggest problem:
      <problem_section(2010, 20, 10, 5)>

      Proof left as an exercise to the reader
    ")
  );
```

As you might have noticed, this idea can be applied to break any text files into modules, not just LaTeX.

---

I'm still figuring out the right syntax so that simple content-only files will
have barely any extra characters. Perhaps an approach is to use the file name as
an identifier so abstract.ptx could simply contain

```
"This problem has been haunting humanity since the dawn of civilization. In this paper, a perfect solution
is introduced."
```

Or even without the quotes...