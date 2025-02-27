import { Slot } from "@radix-ui/react-slot"
import { cva, type VariantProps } from "class-variance-authority"
import { cn, focusRing } from "lib/utils"
import * as React from "react"

const buttonVariants = cva(
  "ml-1px inline-flex items-center justify-center whitespace-nowrap font-medium transition-colors whitespace-nowrap transition-colors focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50",
  {
    compoundVariants: [
      {
        variant: "default",
        color: "muted",
        className: "text-foreground hover:text-background"
      },
      {
        variant: "outline",
        color: "muted",
        className: "text-gray-500"
      },
      {
        variant: ["outline", "default"],
        size: "xl",
        className: "border-3"
      },
      {
        variant: "filled",
        color: "primary",
        className: "bg-primary"
      },
      {
        variant: "filled",
        color: "secondary",
        className: "bg-secondary"
      },
      {
        variant: "filled",
        color: "error",
        className: "bg-error"
      },
      {
        variant: "filled",
        color: "success",
        className: "bg-success"
      },
      {
        variant: "filled",
        color: "warning",
        className: "bg-warning"
      },
      {
        variant: "filled",
        color: "info",
        className: "bg-info"
      },
      {
        asIconButton: true,
        size: "xs",
        className: "w-5"
      },
      {
        asIconButton: true,
        size: "sm",
        className: "w-6"
      },
      {
        asIconButton: true,
        size: "md",
        className: "w-8"
      },
      {
        asIconButton: true,
        size: "lg",
        className: "w-10"
      },
      {
        asIconButton: true,
        size: "xl",
        className: "w-12"
      },
      {
        variant: "ghost",
        color: ["primary", "secondary", "error", "success", "warning", "info"],
        className: "bg-transparent"
      }
    ],
    variants: {
      color: {
        primary:
          "bg-primary/75 border-primary text-primary hover:bg-primary/50 focus:ring-primary",
        secondary:
          "bg-secondary/75 border-secondary text-secondary hover:bg-secondary/50 focus:ring-secondary",
        error:
          "bg-error/75 border-error text-error hover:bg-error/50 focus:ring-error",
        success:
          "bg-success/75 border-success text-success hover:bg-success/50 focus:ring-success",
        warning:
          "bg-warning/75 border-warning text-warning hover:bg-warning/50 focus:ring-warning",
        info: "bg-info/75 border-info text-info hover:bg-info/50 focus:ring-info",
        muted: "border-gray-500 hover:bg-gray-400/50"
      },
      variant: {
        default: "border-2 shadow text-foreground hover:border-foreground",
        filled:
          "shadow text-foreground hover:bg-foreground hover:text-background",
        outline:
          "border-2 shadow bg-transparent hover:text-foreground hover:border-foreground",
        ghost: "shadow hover:bg-accent hover:text-accent-foreground",
        link: "bg-transparent hover:bg-transparent underline focus:ring-offset-0"
      },
      size: {
        xs: "h-5 px-2 text-xs",
        sm: "h-6 px-3 text-sm",
        md: "h-8 px-4 text-base",
        lg: "h-10 px-8 text-lg",
        xl: "h-12 px-10 text-xl"
      },
      round: {
        none: "rounded-none",
        both: "rounded-full",
        left: "rounded-l-full",
        right: "rounded-r-full",
        t: "rounded-t",
        b: "rounded-b",
        tl: "rounded-tl",
        tr: "rounded-tr",
        bl: "rounded-bl",
        br: "rounded-br"
      },
      asIconButton: {
        true: "p-0"
      },
      noShadow: {
        true: "shadow-none"
      }
    },
    defaultVariants: {
      variant: "default",
      color: "primary",
      round: "both",
      size: "md"
    }
  }
)

export interface ButtonProps
  extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "color">,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean
  height?: string
  asIconButton?: boolean
  noShadow?: boolean
  isLoading?: boolean
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      className,
      children,
      color,
      variant,
      round,
      size,
      asIconButton,
      height,
      noShadow,
      isLoading,
      asChild = false,
      ...props
    },
    ref
  ) => {
    const Comp = asChild ? Slot : "button"

    return (
      <Comp
        className={cn(
          focusRing,
          buttonVariants({
            variant,
            color,
            size,
            round,
            asIconButton,
            noShadow
          }),
          className,
          size && size !== "xs" && `rounded-${round}-${size}`,
          height,
          isLoading && "animate-border-pulse"
        )}
        ref={ref}
        disabled={isLoading}
        {...props}
      >
        {children}
      </Comp>
    )
  }
)
Button.displayName = "Button"

export { Button, buttonVariants }
