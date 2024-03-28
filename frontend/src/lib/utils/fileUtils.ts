// fileUtils.ts

import { decodeBase64, encodeBase64 } from '$lib/utils/base64';

export function formatSize(size: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let unitIndex = 0;

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }

  return `${size.toFixed(2)} ${units[unitIndex]}`;
}

export interface FileNode {
  name: string;
  size: number;
  isFile: boolean;
  children?: FileNode[];
}

export function buildFileTree(data: any[], rootPath: string): FileNode[] {
  const decodedData = data.map((item) => ({
    ...item,
    path: decodeBase64(item.path),
    root_path: decodeBase64(item.root_path),
  }));

  const decodedRootPath = decodeBase64(rootPath);

  const rootNode: FileNode = {
    name: decodedRootPath,
    size: 0,
    isFile: false,
    children: [],
  };

  const fileMap: { [key: string]: FileNode } = {};

  decodedData.forEach((item) => {
    const relativePath = item.path.replace(decodedRootPath, '');
    const pathParts = relativePath.split('/').filter((part: string) => part !== '');

    let currentNode = rootNode.children || [];

    pathParts.forEach((part: string, index: number) => {
      let node = currentNode.find((n) => n.name === part);

      if (!node) {
        node = {
          name: part,
          size: 0,
          isFile: false,
          children: [],
        };
        currentNode.push(node);
        fileMap[relativePath] = node;
      }

      if (index === pathParts.length - 1) {
        node.size = item.size;
      }

      currentNode = node.children || [];
    });
  });

  setIsFileProperty(rootNode.children || []);
  calculateFolderSizes(rootNode.children || []);

  return [rootNode];
}

function setIsFileProperty(nodes: FileNode[]) {
  nodes.forEach((node) => {
    if (node.children && node.children.length === 0) {
      node.isFile = true;
    } else {
      node.isFile = false;
      setIsFileProperty(node.children || []);
    }
  });
}

function calculateFolderSizes(nodes: FileNode[]) {
  nodes.forEach((node) => {
    if (!node.isFile) {
      const folderSize = node.children?.reduce((total, child) => total + child.size, 0) || 0;
      node.size = folderSize;
      calculateFolderSizes(node.children || []);
    }
  });
}

export function sortFileTree(nodes: FileNode[], sortBy: 'name' | 'size', sortOrder: 'asc' | 'desc'): FileNode[] {
  const sortedNodes = [...nodes];

  sortedNodes.sort((a, b) => {
    if (sortBy === 'name') {
      return sortOrder === 'asc' ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name);
    } else {
      return sortOrder === 'asc' ? a.size - b.size : b.size - a.size;
    }
  });

  sortedNodes.forEach((node) => {
    if (node.children) {
      node.children = sortFileTree(node.children, sortBy, sortOrder);
    }
  });

  return sortedNodes;
}