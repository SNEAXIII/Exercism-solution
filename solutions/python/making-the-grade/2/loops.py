def round_scores(student_scores):
    return list(map(round, student_scores))

def count_failed_students(student_scores):
    return sum(map(lambda score: score <= 40, student_scores))

def above_threshold(student_scores, threshold):
    return list(filter(lambda x: x >= threshold, student_scores))

def letter_grades(highest):
    step = (highest-40)//4
    return list(map(lambda x: 41 + x * step, range(4)))
    
def student_ranking(student_scores, student_names):
    return [f"{index+1}. {student}: {score}" for index, (score, student) in enumerate(zip(student_scores, student_names))]
    
def perfect_score(student_info):
    return next((info for info in student_info if info[1] == 100), [])