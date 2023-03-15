	.text
	.globl main
main:
	movq $65, %rdi
	movq $1, %r8
	testq %r8, %r8
	jz L83
	movq $1, %r8
	testq %r8, %r8
	jz L83
	movq $1, %r8
L83:
	addq %r8, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $1, %rax
	testq %rax, %rax
	jz L27
	movq $2, %rax
	testq %rax, %rax
	jz L27
	movq $1, %rax
L27:
	addq %rax, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $1, %rcx
	testq %rcx, %rcx
	jz L12
	movq $0, %rcx
	testq %rcx, %rcx
	jz L12
	movq $1, %rcx
L12:
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	jmp L12
	jmp L12
	jmp L27
	jmp L27
	jmp L83
	jmp L83
	.data

