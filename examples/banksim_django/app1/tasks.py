import enum
import hashlib
import json

import requests
from celery import shared_task


class WebHookReq(enum.Enum):
    confirm = 'confirm'
    capture = 'capture'
    cancel = 'cancel'


class WebHookReqEncoder(json.JSONEncoder):
    def default(self, o):
        if isinstance(o, WebHookReq):
            return o.value
        return super().default(o)


@shared_task
def call_webhook(webhook: str, id: str):
    body = request_action(webhook, id, 'terminalpassword')
    url = f'http://localhost:15100/session/{webhook}'
    response = requests.post(url, json=body)
    print(response.json())
    return response.status_code


def request_action(
    webhook: str, session_id: str, cashbox_password: str
) -> dict[str, str]:
    # Create a ConfirmRequest object
    req = {'session_id': str(session_id), 'webhook': webhook.capitalize()}
    req['token'] = generate_token(req, cashbox_password)
    return req


# Generate the token for the request
def generate_token(body, password: str):
    concatenated = ''
    body['password'] = password

    for key in sorted(body.keys()):
        value = body[key]

        if key == 'beneficiaries' and body[key][key]:
            concatenated += ''.join(body[key][key])

        if not isinstance(value, (list, dict)):
            concatenated += str(value)

    # Create a SHA-256 hash of the concatenated string
    hasher = hashlib.sha256()
    hasher.update(concatenated.encode())
    hash_result = hasher.hexdigest()

    # Return the hash result as a hex string
    return hash_result
