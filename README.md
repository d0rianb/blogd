# Blogd

> Writing an article is easy, presenting it correctly is not.

The goal behind this project is to quickly write (well presented) articles and expose them to the Internet.

The site is live at [this url](www.dodax.site).
An explanation of how does this blog works is available [here](https://www.dodax.site/articles/how-this-blog-works). The source of this article is avaible under [articles/how-this-blog-works](./articles/how-this-blog-works.md)

#### Detailed Workflow

```mermaid
flowchart LR

Abase[Markdown article] -->|styled view| A 
Abase -->|raw view| Hpage

subgraph "Backend"
A{"Split"} -->|Parse| M([Metadata])
A ==> |Compile| HA[HTML content]
A -->|Detect| I([Illustration table])
HA ==>|structure adaptation| HAD[HTML Content] ==>|article| H

M -->|Header| H{Combine}
I -->|Footer| H
H --> Hpage[HTML page]
end 
subgraph "Frontend"
Hpage -->|Send| BR[Browser page]
BR --> |"
    CSS styling
    Code highlight
    Equations format
    Graph building
"| FV[Final View]
end 
```

* * *

2023 &copy; Dorian Beauchesne
