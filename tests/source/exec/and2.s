	.text
	.globl main
main:
	movq $65, %rdi
	movq $0, %rcx
	testq %rcx, %rcx
	jz L75
	movq $1, %rcx
	testq %rcx, %rcx
	jz L75
	movq $1, %rcx
L75:
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $0, %r8
	testq %r8, %r8
	jz L67
	movq $2, %r8
	testq %r8, %r8
	jz L67
	movq $1, %r8
L67:
	addq %r8, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $1, %r9
	testq %r9, %r9
	jz L30
	movq $0, %r9
	testq %r9, %r9
	jz L30
	movq $1, %r9
L30:
	addq %r9, %rdi
	call putchar
	addq $0, %rsp
	movq $65, %rdi
	movq $0, %rdx
	testq %rdx, %rdx
	jz L11
	movq $0, %rdx
	testq %rdx, %rdx
	jz L11
	movq $1, %rdx
L11:
	addq %rdx, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	jmp L11
	jmp L11
	jmp L30
	jmp L30
	jmp L67
	jmp L67
	jmp L75
	jmp L75
	.data

