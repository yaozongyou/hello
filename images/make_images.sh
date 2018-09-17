#!/bin/bash

dot -Tpng -o rbd_write_flow.png ./rbd_write_flow.dot
dot -Tpng -o rbd_image_request.png ./rbd_image_request.dot
dot -Tpng -o rbd_object_request.png ./rbd_object_request.dot
