# Installation:
# py -m pip install webdriver-manager
# py -m pip install selenium

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.firefox.options import Options
import time
import json

def load_catalog():
    # try:
        # Run without a GUI.  Note that the script didn't work
        # with Chrome without a GUI.  Firefox was successful.
        options = Options()
        options.add_argument("--headless")
        options.add_argument("--disable-gpu")

        driver = webdriver.Firefox(options=options)
        groups = get_course_groups(driver)
        courses = get_courses(driver, groups)
        courses_json = json.dumps(courses)
        with open("courses.json","w") as file:
            file.write(courses_json)
        
    # except Exception as e:
    #     print(e)

    # finally:
    #     driver.quit()

def get_course_groups(driver):

    print("Getting Course Groups")

    # Read the courses website
    driver.get("https://www.byui.edu/catalog/#/courses")
    time.sleep(2)

    # Find all the course groups from the courses website
    groups = []
    buttons = driver.find_elements(By.XPATH,"//button[contains(@aria-label,'show')]")
    for button in buttons:
        groups.append(button.get_attribute("aria-controls"))

    return groups

def get_courses(driver, groups):
    results = []
    for group in groups:
        print(f"Processing: {group}")
        driver.get(f"https://www.byui.edu/catalog/#/courses/?group={group}")
        time.sleep(2)

        # Find the link for each course.  From the link, we can get the name of the course
        # and the link to the course information
        first_pass_results = []
        courses = driver.find_elements(By.XPATH,"//a[contains(@href, 'bcItemType=courses')]")
        for course in courses:
            course_split = course.text.split(" - ")
            if len(course_split) <= 1:
                print(f"Unexpected: {course.text}")
                continue
            course_id = course_split[0]
            course_title = " - ".join(course_split[1:])
            course_link = course.get_attribute("href")
            course_data = {"id": course_id, "title": course_title, "group": group, "link": course_link}
            first_pass_results.append(course_data)

        for course in first_pass_results:
            print(f"Getting Details: {course["id"]}")
            course_details = get_course_details(driver, course["link"])            
            course.update(course_details)
            results.append(course)

    return results

def get_course_details(driver, course_link):    
    results = {}
    driver.get(course_link)
    time.sleep(2)
    sections = driver.find_elements(By.XPATH,"//div[contains(@class, 'noBreak')]")
    for section in sections:
        section_parts = section.text.split("\n")
        if len(section_parts) <= 1:
            print(f"Unexpected: {section_parts}")
            continue
        section_title = section_parts[0].lower().replace(' ','_')
        if section_parts[1].startswith("keyboard"):
            if len(section_parts) <= 2:
                print(f"Unexpected: {section_parts}")
                continue
            section_content = "\n".join(section_parts[2:])
        else:
            section_content = "\n".join(section_parts[1:])
        
        results[section_title] = section_content
    return results
    

load_catalog()