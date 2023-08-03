

# Blogd


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
