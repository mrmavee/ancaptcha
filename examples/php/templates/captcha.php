<?php
/**
 * @var array{kind: string, diff: string, css: string, html: string, token: string, status_html: string, show_form: bool, error_msg: string} $view_data
 */
$kind = $view_data['kind'];
$diff = $view_data['diff'];
$css = $view_data['css'];
$html = $view_data['html'];
$token = $view_data['token'];
$status_html = $view_data['status_html'];
$show_form = $view_data['show_form'];
?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title><?php echo $kind; ?> - <?php echo $diff; ?></title>
    <style>
        <?php echo $css; ?>
        body { margin: 0; min-height: 100vh; background: #f0f2f5; font-family: sans-serif; padding: 20px; box-sizing: border-box; display: flex; align-items: center; justify-content: center; }
        .container { width: 100%; max-width: 500px; padding: 40px 30px 30px; background: #fff; border-radius: 12px; box-shadow: 0 10px 30px rgba(0,0,0,0.05); box-sizing: border-box; }
        h2 { margin-top: 0; text-align: center; color: #333; }
        textarea { width: 100%; height: 100px; padding: 12px; border: 1px solid #ddd; border-radius: 4px; margin-bottom: 30px; font-family: inherit; box-sizing: border-box; resize: vertical; }
        .submit-btn { background: #2c3e50; color: white; border: none; padding: 12px 24px; border-radius: 4px; cursor: pointer; font-size: 16px; font-weight: bold; width: 100%; transition: all 0.2s; user-select: none; margin-top: 10px; }
        .submit-btn:hover { background: #34495e; transform: translateY(-1px); box-shadow: 0 4px 8px rgba(0,0,0,0.1); }
        .submit-btn:active { transform: translateY(0) scale(0.98); }
    </style>
</head>
<body>
    <div class="container">
        <h2>Post a Comment</h2>
        <?php echo $status_html; ?>
        
        <?php if ($show_form): ?>
        <form method="post">
            <textarea name="comment" placeholder="Write comment...">Simulated comment.</textarea>
            <?php echo $html; ?>
            <div style="margin-top:20px;">
                <button type="submit" class="submit-btn">Submit Comment</button>
            </div>
        </form>
        <?php else: ?>
        <div style="text-align:center;padding:20px;background:#f8f9fa;border-radius:4px;border:1px solid #dee2e6;">
            The comment has been submitted.
        </div>
        <?php endif; ?>
        
        <p style="margin-top:20px;text-align:center;"><a href="?" style="color:#666;text-decoration:none;">&larr; Back to Index</a></p>
    </div>
</body>
</html>
