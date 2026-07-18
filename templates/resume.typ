#set page(paper: "us-letter", margin: (x: 1.5cm, top: 2cm, bottom: 2.5cm))
#set text(font: "Roboto", size: 11pt, fill: rgb("#2d3748"))

// Header Section
#align(center)[
  #text(size: 26pt, weight: "bold", fill: rgb("#1a365d"))[{{ name }}] \
  #v(2mm)
  #text(size: 10pt, fill: rgb("#4a5568"))[
    #("{{ email }}") | #("{{ phone }}")
  ]
]

#v(1.5em)

// Experience Section
#text(size: 14pt, weight: "bold", fill: rgb("#1a365d"))[Professional Experience]
#v(-2mm)
#line(length: 100%, stroke: 0.5pt + rgb("#cbd5e0"))
#v(0.5em)

{% for job in jobs %}
#grid(
  columns: (1fr, auto),
  [*{{ job.role }}* \ _ {{ job.company }} _],
  [{{ job.dates }}]
)
#v(0.2em)
- {{ job.description }}
#v(1em)
{% endfor %}