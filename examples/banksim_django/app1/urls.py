from django.urls import path
from . import views

app_name = 'app1'

urlpatterns = [
    path('notification', views.post_notification, name='post_notification'),
    path('success', views.success_page, name='success_page'),
    path('fail', views.fail_page, name='fail_page')
]
