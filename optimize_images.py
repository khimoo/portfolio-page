#!/usr/bin/env python3
"""
画像最適化スクリプト
ノードの物理演算パフォーマンス向上のため、author_img.pngを最適化します。
"""

from PIL import Image
import os

def optimize_author_image():
    input_path = "khimoo-portfolio/articles/img/author_img.png"
    output_dir = "khimoo-portfolio/articles/img/"
    
    if not os.path.exists(input_path):
        print(f"Error: {input_path} not found")
        return
    
    # 元画像を読み込み
    with Image.open(input_path) as img:
        print(f"Original size: {img.size}, format: {img.format}")
        
        # 64x64の小さいバージョンを作成（ノード用）
        small_img = img.resize((64, 64), Image.Resampling.LANCZOS)
        small_path = os.path.join(output_dir, "author_img_small.png")
        small_img.save(small_path, "PNG", optimize=True)
        print(f"Created small version: {small_path} ({os.path.getsize(small_path)} bytes)")
        
        # WebP形式でも保存（さらに小さく）
        webp_path = os.path.join(output_dir, "author_img_small.webp")
        small_img.save(webp_path, "WEBP", quality=85, optimize=True)
        print(f"Created WebP version: {webp_path} ({os.path.getsize(webp_path)} bytes)")
        
        # 128x128の中サイズも作成（将来の拡張用）
        medium_img = img.resize((128, 128), Image.Resampling.LANCZOS)
        medium_path = os.path.join(output_dir, "author_img_medium.png")
        medium_img.save(medium_path, "PNG", optimize=True)
        print(f"Created medium version: {medium_path} ({os.path.getsize(medium_path)} bytes)")

if __name__ == "__main__":
    try:
        optimize_author_image()
        print("Image optimization completed successfully!")
    except ImportError:
        print("PIL (Pillow) not available. Please install with: pip install Pillow")
    except Exception as e:
        print(f"Error: {e}")