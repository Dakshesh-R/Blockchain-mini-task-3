# Blockchain-mini-task-3
# Student Report Card – Rust Console App
This Rust-based console application takes student details from the user (name, type of CRUD operation ,individual marks), calculates the average, assigns a grade, and generates a clean PDF report card.

## Features
- Accepts student input via console
- Calculates average marks
- Assigns grades based on average:
  - A: 90+
  - B: 75–89
  - C: 60–74
  - D: Below 60
- Generates a neat PDF report card using printpdf

## Features
- Add a new student and their marks for 5 subjects
- Automatically calculate total, average, and assign a grade (A to D)
- View an individual student's report
- Update an existing student's marks
- Delete a student's record
- List all saved students

## Technical Details
- Data is stored in-memory using a `HashMap`
- Uses `struct` for student data modeling
