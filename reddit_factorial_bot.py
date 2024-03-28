# let it be said that I hate the python prettifier

import praw
import os
import re
import subprocess

reddit = praw.Reddit(
    user_agent=os.environ.get("FACTORIAL_BOT_USER_AGENT"),
    client_id=os.environ.get("FACTORIAL_BOT_CLIENT_ID"),
    client_secret=os.environ.get("FACTORIAL_BOT_CLIENT_SECRET"),
    username=os.environ.get("FACTORIAL_BOT_USERNAME"),
    password=os.environ.get("FACTORIAL_BOT_PASSWORD")
)


def send_reply(parent_comment, message):
    parent_comment.reply(
        f"{message}\n\n^this ^action ^was ^performed ^by "
        "^the ^factorial ^calculator ^bot, "
        "^if ^you ^have ^any ^questions ^contact ^the "
        "^[developer](https://www.reddit.com/u/GDOR-11)"
    )


subreddit = reddit.subreddit("unexpectedfactorial+factorialchain")
for comment in subreddit.stream.comments():
    result = re.search("(\\d+)!", comment.body)
    if result is None:
        continue

    target = int(result.group(1))
    if target > 100000000:
        send_reply(comment, "this number is too large to calculate :(")
        continue

    result = subprocess.run(["cargo", "run", "--", f"target={target}"],
                            stdout=subprocess.DEVNULL,
                            stderr=subprocess.STDOUT)

    if result.returncode != 0:
        send_reply(comment, "this number is too large to calculate :(")

    send_reply(comment, f"[{target}!]"
               "(https://gdor-11.github.io/big-numbers/view-number"
               f"?number={target}!)")
