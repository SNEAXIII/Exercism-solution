from datetime import timedelta, datetime
def add(moment: datetime) -> datetime: 
    return moment + timedelta(seconds=10**9)