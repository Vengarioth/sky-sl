# sky-sl grammar

A formal specification for sky-sl syntax.

## legend

|    token     | meaning                       |
| :----------: | ----------------------------- |
|   **Rule**   | definition of a rule          |
|    *Rule*    | reference to a rule           |
|   `token`    | a specific token              |
|      \|      | or                            |
|      (       | begin group                   |
|      )       | end group                     |
| <sup>?</sup> | the previous item is optional |

## top level items
---

### module declaration

**ModuleDeclaration:**<br>
&nbsp;&nbsp;&nbsp;&nbsp;`mod` *Identifier* `;`

---

### use declaration

**UseDeclaration:**<br>
&nbsp;&nbsp;&nbsp;&nbsp;`use` *UseTree* `;`

**UseTree:**<br>
&nbsp;&nbsp;&nbsp;&nbsp;*UseSegment*<br>
&nbsp;&nbsp;&nbsp;&nbsp;| *UseSegment* `::` `*`<br>
&nbsp;&nbsp;&nbsp;&nbsp;| *UseSegment* `::` *UseTree*<br>
&nbsp;&nbsp;&nbsp;&nbsp;| *UseSegment* `::` *UseGroup*<br>

**UseGroup**<br>
&nbsp;&nbsp;&nbsp;&nbsp;`{` *UseTree* ( `,` *UseTree* )<sup>?</sup> `,`<sup>?</sup> `}`<br>

**UseSegment**<br>
&nbsp;&nbsp;&nbsp;&nbsp;*Identifier* | `package` | `super`<br>

---

### function definition

> *TODO*

---

### struct definition

> *TODO*
