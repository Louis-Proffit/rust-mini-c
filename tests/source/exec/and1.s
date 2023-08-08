	.text
	.globl main
main:
	movq $65, %rdi
	movq $1, %r9
	testq %r9, %r9
	jz L44
	movq $1, %r9
	testq %r9, %r9
	jz L44
	movq $1, %r9
L44:
	addq %r9, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rax
	testq %rax, %rax
	jz L36
	movq $2, %rax
	testq %rax, %rax
	jz L36
	movq $1, %rax
L36:
	addq %rax, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %rdx
	testq %rdx, %rdx
	jz L28
	movq $0, %rdx
	testq %rdx, %rdx
	jz L28
	movq $1, %rdx
L28:
	addq %rdx, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	jmp L28
	jmp L28
	jmp L36
	jmp L36
	jmp L44
	jmp L44
	.data

