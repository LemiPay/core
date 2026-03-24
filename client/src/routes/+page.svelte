<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Lemipay - Home</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        }

        body {
            background-color: #ffffff;
            height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }

        .container {
            width: 100%;
            max-width: 400px;
            padding: 2.5rem;
            border: 1px solid #f0f0f0;
            border-radius: 12px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
            text-align: left;
        }

        h1 {
            font-size: 1.8rem;
            font-weight: 800;
            margin-bottom: 0.5rem;
            color: #000;
        }

        p {
            font-size: 0.95rem;
            color: #666;
            margin-bottom: 2rem;
            line-height: 1.4;
        }

        .button-group {
            display: flex;
            flex-direction: column;
            gap: 1rem;
        }

        .btn {
            display: block;
            width: 100%;
            padding: 0.85rem;
            text-align: center;
            text-decoration: none;
            font-weight: 600;
            border-radius: 8px;
            transition: opacity 0.2s;
        }

        .btn-primary {
            background-color: #000;
            color: #fff;
        }

        .btn-secondary {
            background-color: #fff;
            color: #000;
            border: 1px solid #e0e0e0;
        }

        .btn:hover {
            opacity: 0.8;
        }
    </style>
</head>
<body>

<div class="container">
    <h1>Lemipay</h1>
    <p>Manage your group expenses in a decentralized and transparent way.</p>

    <div class="button-group">
        <a href="/login" class="btn btn-primary">Log in</a>
        <a href="/register" class="btn btn-secondary">Create account</a>
    </div>
</div>

</body>
</html>