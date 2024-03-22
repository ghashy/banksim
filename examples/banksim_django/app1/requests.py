import requests
from app1.tasks import generate_token


def make_payment(amount: int):
    body: dict = {
        'notification_url': 'http://localhost:8000/app1/notification',
        'success_url': 'http://localhost:8000/app1/success',
        'fail_url': 'http://localhost:8000/app1/fail',
        'amount': amount,
        'beneficiaries': {'beneficiaries': []},
    }
    token = generate_token(body, 'terminalpassword')
    print(token)
    body['token'] = token

    response = requests.post(
        'http://localhost:15100/session/init/payment', json=body
    )
    print(response.json())


def reg_token():
    body: dict = {
        'notification_url': 'http://localhost:8000/app1/notification',
        'success_url': 'http://localhost:8000/app1/success',
        'fail_url': 'http://localhost:8000/app1/fail',
    }
    token = generate_token(body, 'terminalpassword')
    print(token)
    body['token'] = token

    response = requests.post(
        'http://localhost:15100/session/init/card_token_reg', json=body
    )
    print(response.json())
