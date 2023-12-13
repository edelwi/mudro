authors = {}
quotes = {}
author_id = 1
quote_id = 1

ins_authors = []
ins_quotes = []
with open('init_quotes_raw.txt', 'r') as f:
    for ln in f.readlines():
        line = ln.strip('\n').strip()
        # print(line)
        splited = line.split(' - ')
        quote = splited[0].strip('"')
        author = splited[1]
        # print(f"{quote} {author}")
        a_id = authors.setdefault(author, author_id)
        
        if a_id == author_id:
            # a = f"INSERT INTO author (id, author_name) VALUES ({a_id}, '{author}');"
            a = f"INSERT INTO author (author_name) VALUES ('{author}');"

            ins_authors.append(a)
            # print(a)
            author_id += 1
        else:
            # print(f"!!! Author exists {author} {a_id}")
            pass

        q_id = quotes.setdefault(quote, quote_id)
        if q_id == quote_id:
            # q = f"INSERT INTO quote (id, text, author_id) VALUES ({q_id}, $$'{quote}'$$, {a_id});"
            q = f"INSERT INTO quote (text, author_id) VALUES ($$'{quote}'$$, {a_id});"

            ins_quotes.append(q)
            # print(q)
            quote_id += 1
        else:
            # print(f"=== Repeated quote {quote}")
            pass

for a in ins_authors:
    print(a)
print("COMMIT;")

for q in ins_quotes:
    print(q)
print("COMMIT;")