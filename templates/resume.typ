// Configure document geometry and styling
#set page(paper: "us-letter", margin: (x: 1.5cm, top: 2cm, bottom: 2.5cm))
#set text(font: "Liberation Sans", size: 10pt, fill: rgb("#2d3748"))

// Header Section
#align(center)[
  #text(size: 24pt, weight: "bold", fill: rgb("#1a365d"))[{{ name }}] \
  #text(size: 10pt, fill: rgb("#4a5568"))[{{ email }} | {{ phone }}]
]

#v(1em)

// Experience Section
#text(size: 14pt, weight: "bold", fill: rgb("#1a365d"))[Professional Experience]
#line(length: 100%, stroke: 0.5pt + rgb("#cbd5e0"))

{% for job in jobs %}
#grid(
  columns: (1fr, auto),
  [*{{ job.role }}* \ _ {{ job.company }} _],
  [{{ job.dates }}]
)
#v(0.2em)
- {{ job.description }}
#v(0.8em)
{% endfor %}