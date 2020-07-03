# Nikel-CLI

A sample CLI program for querying the UofT API Nikel: https://nikel.ml

Using the library [nikel-rs](https://github.com/George-lewis/Nikel-rs)

# Usage

Enter commands into the prompt, commands take the form:

`[type] [attribute]:(operator)[value],[...]`

where `(operator)` is optional and is any of the operators described at: https://docs.nikel.ml/docs/query_guide#advanced-queries

where `[type]` is any of:
- courses, classes
- textbooks, tb
- exams
- eval
- food
- services, serv
- parking, park

# Example Queries

`courses code:csc108,campus:mis`
> Query all courses with `csc108` in the `code` (e.g. `CSC108H1`) and `mis` in the `campus` (e.g. `mississauga`)

`tb title:algebra,price:>100`
> Query all algebra textbooks with a price over 100 dollars
