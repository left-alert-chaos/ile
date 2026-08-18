"""
# display.py
This module reads input and formats it in a Tkinter window.
It is embedded in VeBaGu.
"""

import tkinter as tk
from tkinter.font import Font
import threading
import sys


def listen():
    global widget_text
    print("listening...")
    failed_to_read = False
    while True:
        # if you can't read, just loop indefinitely. It'll be fiiiine.
        try:
            text = input().strip()
        except:
            if not failed_to_read:
                print("Failed to read!")
            failed_to_read = True
            continue

        if text == "":
            continue
        print(text)

        # get command
        words = text.split()
        command = words.pop(0).upper()
        text = ' '.join(words)
        
        match command:
            case "QUIT":
                root.destroy()
                return
            case "H1":
                tk.Label(root, text=text, font=h1).pack()
                print("added h1")
            case "H2":
                tk.Label(root, text=text, font=h2).pack()
            case "H3":
                tk.Label(root, text=text, font=h3).pack()
            case "H4":
                tk.Label(root, text=text, font=h4).pack()
            case "P":
                tk.Label(root, text=text, font=p).pack()


def mainloop():
    try:
        root.mainloop()
    except RuntimeError as e:
        print(f"INFO: error while looping window: {e}")


widget_text = "placeholder"

root = tk.Tk()
h1 = Font()
h1.configure(size=30)
h2 = Font()
h2.configure(size=27)
h3 = Font()
h3.configure(size=22)
h4 = Font()
h4.configure(size=18)
p = Font()

root.event_generate

threading.Thread(target=mainloop, name="window").start()
listen()

