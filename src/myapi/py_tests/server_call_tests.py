import requests
import json
import time

#for now each test asks for a new token

base_url = "http://192.168.0.208:8000/"


def login(admin=True):
    data = json.dumps({
        "username": "admin",
        "password": "admin"
    }) if admin else json.dumps({})
    headers = {'Content-Type': 'application/json'}
    url = base_url + "login"
    response = requests.post(url, data=data, headers = headers)
    if response.status_code == 200:
        try:
            return response.json()['token']
        except (ValueError, KeyError):
            print("Error parsing token from response.")
    else:
        print(f"Error logging in: {response.status_code}")
        print(response.text)

def get_habits(token):
        url = base_url+"habits"
        base_headers = {}
        base_headers['Authorization']= f"Bearer {token}"
        response = requests.get(url, headers = base_headers)
        print(response.json())

def get_users(token):
    url = base_url+"users"
    base_headers = {'Authorization': f'Bearer {token}'}
    response = requests.get(url, headers = base_headers)
    print(response.json())
    
def create_habit(user_id, name, token):
    base_headers = {}
    base_headers['Authorization']= f"Bearer {token}"
    base_headers["Content-Type"] = "application/json"
    url = base_url+f"users/{user_id}/habits"
    data = json.dumps({
        "user_id": user_id,
        "name": name
    })
    print(data)
    response = requests.post(url = url, headers = base_headers, data = data )
    print(response.json())
    
def create_user(name, email, role, password, token):
    base_headers = {}
    base_headers['Authorization']= f"Bearer {token}"
    base_headers["content-type"] = "application/json"
    url = base_url+f"users/"
    data = json.dumps({
        "name": name,
        "email": email,
        "role": role,
        "passhash": password,
        
    })
    print(data)
    response = requests.post(url = url, headers = base_headers, data = data)
    print(response.json())
    
def update_habit(habit_user_id, habit_id, new_name, token):
    base_headers = {}
    base_headers['Authorization']= f"Bearer {token}"
    base_headers["content-type"] = "application/json"
    url = base_url+f"users/{habit_user_id}/habits/{habit_id}"
    #data = {}
    #data["user_id"] = habit_user_id
    #data["name"] = new_name
    data = json.dumps({
        "user_id": habit_user_id,
        "name": new_name
    })
    print(data)
    response = requests.put(url = url, headers = base_headers, data = data)
    
def view_habit(habit_user_id, habit_id, token):
    base_headers = {}
    base_headers['Authorization']= f"Bearer {token}"
    base_headers["content-type"] = "application/json"
    url = base_url+f"users/{habit_user_id}/habits/{habit_id}"
    response = requests.get(url=url, headers = base_headers)
    print(response.json())

def delete_habit(habit_user_id, habit_id, token):

    base_headers = {}
    base_headers['Authorization']= f"Bearer {token}"
    base_headers["content-type"] = "application/json"
    url = base_url+f"users/{habit_user_id}/habits/{habit_id}"
    response = requests.delete(url=url, headers = base_headers)
    print(response.json())
    
    
    
    
def update_user(user_id, token, new_name=None, email=None):
    base_headers = {
        'Authorization': f"Bearer {token}",
        "content-type": "application/json"
    }
    url = base_url + f"users_controller/{user_id}"
    data = {}
    if new_name is not None:
        data["name"] = new_name
    if email is not None:
        data["email"] = email
    response = requests.put(url=url, headers=base_headers, data=json.dumps(data))
    print(response.json())

def delete_user(id, token):
    base_headers = {
        'Authorization': f"Bearer {token}",
        "content-type": "application/json"
    }
    url = base_url + f"users/{id}"
    response = requests.delete(url = url, headers = base_headers)
    print(response.json())

def view_user(id, token):
    base_headers = {
        'Authorization': f"Bearer {token}",
        "content-type": "application/json"
    }
    url = base_url + f"users/{id}"
    response = requests.get(url = url, headers = base_headers)
    print(response.json())
    
    def throughput_testing(token):
        success_count = 0
        total_count = 0
        start_time = time.time()
        
        while time.time() - start_time < 60:  # Run the test for 60 seconds
            try:
                create_habit(1, "test", token)
                success_count += 1
            except Exception as e:
                print(f"Error creating habit: {e}")
            total_count += 1
        
        success_rate = success_count / total_count
        return success_rate

def create_task(user_id, habit_id, task_name, token):
        base_headers = {}
        base_headers['Authorization'] = f"Bearer {token}"
        base_headers["Content-Type"] = "application/json"
        url = base_url + f"users/{user_id}/tasks"
        data = json.dumps({
            "user_id": user_id,
            "name": task_name,
            "habit_id": habit_id
        })
        response = requests.post(url=url, headers=base_headers, data=data)
        print(response.json())

def view_task(user_id, task_id, token):
    base_headers = {
        'Authorization': f"Bearer {token}",
        "content-type": "application/json"
        }
    url = base_url + f"users/{user_id}/tasks/{task_id}"
    response = requests.get(url=url, headers=base_headers)
    print(response.json())
    



def update_task(user_id, habit_id, task_id, new_name, token):
        base_headers = {
            'Authorization': f"Bearer {token}",
            "content-type": "application/json"
        }
        url = base_url + f"users/{user_id}/tasks/{task_id}"
        data = json.dumps({
            "name": new_name,
            "habit_id": habit_id
        })
        response = requests.put(url=url, headers=base_headers, data=data)
        print(response.json())

def main():
    token = login()
    #get_habits()
    
    #create_habit(1, "test", token)
    
    
    get_users()
    #create_habit(1, "test")
    #create_user("test", "test", "user", "test")
    #update_habit(1, 1, "test2")
    #view_habit(1, 1)
    #delete_habit(1, 1)
    #update_user(1, "test2")
    #delete_user(1)
    view_user(1)