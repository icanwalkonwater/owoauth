install-git-hooks:
	pre-commit install
	pre-commit install -t pre-commit -t commit-msg
