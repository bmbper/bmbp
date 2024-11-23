import base64
import sys

def image_to_base64(image_path):
   with open(image_path, "rb") as image_file:
       encoded_string = base64.b64encode(image_file.read()).decode('utf-8')
       return encoded_string
if __name__ == "__main__":
   if len(sys.argv) != 2:
       print("Usage: python image_to_base64.py <image_path>")
       sys.exit(1)
   image_path = sys.argv[1]
   base64_string = image_to_base64(image_path)
   print(base64_string)
