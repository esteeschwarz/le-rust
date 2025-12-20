# Diagramme généré depuis input.md

```mermaid
graph TD
    TA[target group]
    WH[who writes]
    WI[with what capacities]
    TE[technical]
    WI --> TE
    SC[scientifically]
    WI --> SC
    TI[time effort]
    WI --> TI
    WH2[whats her use purpose of the center page]
    LA[landing page for individually set up project page]
    WH2 --> LA
    WI2[with only project abstract]
    WH2 --> WI2
    FO[2-fold like now, with distinct project pages behind landingpage which could all be rendered differently individually, according to the framework used by the project maintainers and just needed to be pushed to main page]
    WH2 --> FO
    CO[complete project presentation]
    WH2 --> CO
    NE[needs to implement complex built paper?]
    WH2 --> NE
    ST[standard paper layout?]
    WH2 --> ST
    BU[built-in pdf export extra to online version  -- quarto, rmarkdown, python notebooks]
    ST --> BU
    US[user interaction needed?]
    WH2 --> US
    BL[blog format]
    WH2 --> BL
    NU[number of metadata to be included and how displayed?]
    WH2 --> NU
    XM[x meta from .csv table, layout template to include preformatted display fields for metadata.]
    NU --> XM
```

## Source originale

- target group
- who writes
	- with what capacities
		- technical
		- scientifically
		- time effort
	- whats her use purpose of the center page
		- landing page for individually set up project page
			- with only project abstract
			- 2-fold like now, with distinct project pages behind landingpage which could all be rendered differently individually, according to the framework used by the project maintainers and just needed to be pushed to main page
		- complete project presentation
			- needs to implement complex built paper?
			- standard paper layout?
			  - built-in pdf export extra to online version  -- quarto, rmarkdown, python notebooks
		- user interaction needed?
			- blog format
		- number of metadata to be included and how displayed?
		  - x meta from .csv table, layout template to include preformatted display fields for metadata.