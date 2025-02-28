import fitz  # PyMuPDF

def read_pdf_lines(pdf_path):
    # Open the PDF file
    document = fitz.open(pdf_path)
    lines = []

    # Iterate over each page
    for page_num in range(len(document)):
        # Get the page
        page = document[page_num]
        # Extract the text of the page
        text = page.get_text("text")
        # Split the text into lines and add to the list
        lines.extend(text.splitlines())
    
    return lines

# Path to your PDF file
pdf_path = "da5.pdf"

# Read the lines from the PDF
pdf_lines = read_pdf_lines(pdf_path)

courses_taken = {}

# Print the lines
for line in pdf_lines:
    if len(line) >= 84: 
        code = line[1:6].strip()
        course = line[55:68].strip().replace(" ","")
        term = line[68:72].strip()
        credit = line[77:81].strip()
        grade = line[83:].strip()

        try:
            credit = float(credit)
        except:
            continue

        if course not in courses_taken:
            courses_taken[course] = ([code], course, [term], [credit], [grade])

        else:
            if course.endswith("R"):
                (prev_codes,_,prev_terms,prev_credits,prev_grades) = courses_taken[course]
                if term not in prev_terms:
                    courses_taken[course] = (prev_codes+[code], course, prev_terms+[term], 
                                             prev_credits+[credit], prev_grades+[grade])


total_credits = 0
in_progress_credits = {}
for course in sorted(courses_taken.keys()):
    print(courses_taken[course])
    (codes, course, terms, credits, grades) = courses_taken[course]
    for i in range(len(terms)):
        code = codes[i]
        term = terms[i]
        credit = credits[i]
        grade = grades[i]
        if code not in ['*NW','*SW','*TW']:
            if grade in ['D-','D','D+','C-','C','C+','B-','B','B+','A-','A','P']: 
                total_credits += credit
            elif grade in ['IP','IPR']:
                if term in in_progress_credits:
                    in_progress_credits[term] += credit
                else:
                    in_progress_credits[term] = credit

print(f"Total Credits: {total_credits}")
print(f"In Progress Credits: {in_progress_credits}")


