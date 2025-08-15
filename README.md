# Email newsletter
With this project I will try to implement Email newsletter from zero, following the book "Zero To Production In Rust".

## User stories to implement

We will fulfill three user stories:

- [ ] As a blog **visitor**,
I want to subscribe to the newsletter,
So that I can receive email updates when new content is published on the blog;
- [ ] As the blog **author**,
I want to send an email to all my subscribers,
So that I can notify them when new content is published;
- [ ] As a **subscriber**,
I want to be able to unsubscribe from the newsletter,
So that I can stop receiving email updates from the blog.

We will not add features to

- manage multiple newsletters;
- segment subscribers in multiple audiences;
- track opening and click rates.


## Technical stuff

### API

```
/health_check GET

/subscriptions POST
```


