import os
import re
import xml.etree.ElementTree as ET
from datetime import datetime

def parse_metadata(md_content):
    metadata = {}
    metadata_pattern = re.compile(r"--\s*(\w+):\s*(.+)")
    for match in metadata_pattern.finditer(md_content):
        key, value = match.groups()
        metadata[key.lower()] = value.strip()
    return metadata

def format_date(date_str):
    """
    Expects date_str in format YYYY-MM-DD.
    Returns date formatted as RFC 2822 (e.g., Wed, 02 Feb 2025 00:00:00 GMT).
    """
    try:
        dt = datetime.strptime(date_str, "%Y-%m-%d")
        return dt.strftime("%a, %d %b %Y %H:%M:%S GMT")
    except ValueError:
        return date_str  # if parsing fails, return original string

def create_rss_feed(blog_posts, output_file="rss.xml"):
    rss = ET.Element("rss", version="2.0")
    channel = ET.SubElement(rss, "channel")
    ET.SubElement(channel, "title").text = "Regalk's RSS Feed"
    ET.SubElement(channel, "link").text = "https://regalk.dev"
    ET.SubElement(channel, "description").text = "Latest blog posts"
    ET.SubElement(channel, "lastBuildDate").text = datetime.utcnow().strftime("%a, %d %b %Y %H:%M:%S GMT")

    for post in blog_posts:
        item = ET.SubElement(channel, "item")
        title = post.get("title", "Untitled")
        link = f"https://regalk.dev/blogs/{post['file']}"
        description = post.get("desc", "")
        raw_date = post.get("date", "")
        pub_date = format_date(raw_date) if raw_date else ""

        ET.SubElement(item, "title").text = title
        ET.SubElement(item, "link").text = link
        ET.SubElement(item, "description").text = description
        ET.SubElement(item, "pubDate").text = pub_date
        
        # Add guid element; here we use the link as a unique identifier
        guid = ET.SubElement(item, "guid")
        guid.text = link
        guid.set("isPermaLink", "true")
        
    tree = ET.ElementTree(rss)
    tree.write(output_file, encoding="utf-8", xml_declaration=True)

def main():
    blog_folder = "blogs"
    blog_posts = []
    
    for filename in os.listdir(blog_folder):
        if filename.endswith(".md"):
            with open(os.path.join(blog_folder, filename), "r", encoding="utf-8") as f:
                content = f.read()
                metadata = parse_metadata(content)
                metadata["file"] = filename
                blog_posts.append(metadata)
    
    create_rss_feed(blog_posts)
    print("RSS feed generated successfully!")

if __name__ == "__main__":
    main()

