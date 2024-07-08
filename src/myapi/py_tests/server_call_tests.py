import requests
import json

#for now each test asks for a new token

base_url = "http://127.0.0.1:8000/"


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

def get_habits():
        url = base_url+"habits"
        token = login(admin=1)
        #print(token)
        base_headers = {}
        base_headers['Authorization']= f"Bearer {token}"
        response = requests.get(url, headers = base_headers)
        print(response.json())

def get_users():
    url = base_url+"users"
    token = login(admin=True)
    print(token)
    base_headers = {'Authorization': f'Bearer {token}'}
    print(base_headers)
    response = requests.get(url, headers = base_headers)
    print(response.json())
    
def create_habit(user_id, name):
    token = login(admin=1)
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
    
def create_user(name, email, role, password):
    token = login(admin=1)
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
    
def update_habit(habit_user_id, habit_id, new_name):
    token = login(admin=1)
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
    
def view_habit(habit_user_id, habit_id):
    token = login(admin=1)
    base_headers = {}
    base_headers['Authorization']= f"Bearer {token}"
    base_headers["content-type"] = "application/json"
    url = base_url+f"users/{habit_user_id}/habits/{habit_id}"
    response = requests.get(url=url, headers = base_headers)
    print(response.json())
    
    
    
def update_user(user_id, new_name=None, email=None):
    token = login(admin=1)
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

def delete_user(id):
    token = login(admin=1)
    base_headers = {
        'Authorization': f"Bearer {token}",
        "content-type": "application/json"
    }
    url = base_url + f"users/{id}"
    response = requests.delete(url = url, headers = base_headers)
    print(response.json())
    
    