import json

from django.http import HttpResponse, HttpRequest
from django.views.decorators.csrf import csrf_exempt
from django.views.decorators.http import require_http_methods

from .tasks import WebHookReq, call_webhook


@csrf_exempt
@require_http_methods(['POST'])
def post_notification(request: HttpRequest) -> HttpResponse:
    ok = HttpResponse(status=200)
    # Get the JSON dictionary from the request body
    json_data = json.loads(request.body.decode(request.encoding or 'utf-8'))

    if 'PaymentNotification' in json_data:
        handle_payment_notification(json_data['PaymentNotification'])
    elif 'TokenNotification' in json_data:
        handle_token_notification(json_data['TokenNotification'])

    return ok


def handle_payment_notification(body: dict):
    if 'ReadyToConfirm' in body.keys():
        webhook = WebHookReq.confirm
        id = body['ReadyToConfirm']['session_id']
    elif 'ReadyToCapture' in body.keys():
        webhook = WebHookReq.capture
        id = body['ReadyToCapture']['session_id']
    elif 'PaymentFinished' in body.keys():
        status = body['PaymentFinished']['status']
        print(f'Payment finished: {status}')
        return

    call_webhook.apply_async(args=[webhook.value, id], countdown=0.7)


def handle_token_notification(body: dict):
    if 'ReadyToConfirm' in body.keys():
        webhook = WebHookReq.confirm
        id = body['ReadyToConfirm']['session_id']
    elif 'Finished' in body.keys():
        status = body['Finished']['status']
        card_token = body['Finished'].get('card_token', 'no_token')
        print(f'Card token reg finished: {status}, token: {card_token}')
        return

    call_webhook.apply_async(args=[webhook.value, id], countdown=0.7)


def success_page(__request__):
    return HttpResponse(b'Success!')


def fail_page(__request__):
    return HttpResponse(b'Fail!')
