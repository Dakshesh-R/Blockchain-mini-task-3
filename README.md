# Blockchain-mini-task-3
# Student Report Card – Rust Console App
This Rust-based console application takes student details from the user (name, total marks, and number of subjects), calculates the average, assigns a grade, and generates a clean PDF report card.

## Features
- Accepts student input via console
- Calculates average marks
- Assigns grades based on average:
  - A: 90+
  - B: 75–89
  - C: 60–74
  - D: Below 60
- Generates a neat PDF report card using printpdf

# Student Report Card System

This is a simple console application written in Rust for managing student report cards.  
It is designed for beginners and demonstrates key Rust concepts in a clear, minimal way.

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
