
# rbd

## 类之间的关系

![rbd image request](images/rbd_image_request.png)

在`AbstractImageWriteRequest<I>::send_object_requests`这个函数中，会把image
request的请求转化成对rados层的object的request的请求，object request相关类
的关系如下：

![rbd object request](images/rbd_object_request.png)
